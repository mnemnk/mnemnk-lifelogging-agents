use std::io::Cursor;
use std::time::Instant;

use anyhow::Result;
use base64::{self, Engine as _};
use chrono::{DateTime, Utc};
use clap::Parser;
use image::{GrayImage, ImageFormat, RgbaImage};
use serde_json::Value;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};
use tokio::signal::ctrl_c;
use tokio::time;
use xcap::Monitor;

const AGENT_NAME: &str = "mnemnk-screen";
const KIND: &str = "screen";

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct AgentConfig {
    /// Interval in seconds
    interval: u64,

    /// Each RGB value is considered as black if it is less than this value
    almost_black_threshold: u64,

    /// Number of non-blank pixels to consider the screen as non-blank
    non_blank_threshold: u64,

    /// Ratio of different pixels to consider the screen as the same
    same_screen_ratio: f32,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            interval: 60,
            almost_black_threshold: 20,
            non_blank_threshold: 400,
            same_screen_ratio: 0.01,
        }
    }
}

impl From<&str> for AgentConfig {
    fn from(s: &str) -> Self {
        let mut config = AgentConfig::default();
        if let Value::Object(c) = serde_json::from_str(s).unwrap_or(Value::Null) {
            if let Some(interval) = c.get("interval") {
                config.interval = interval.as_u64().unwrap();
            }
            if let Some(almost_black_threshold) = c.get("almost_black_threshold") {
                config.almost_black_threshold = almost_black_threshold.as_u64().unwrap();
            }
            if let Some(non_blank_threshold) = c.get("non_blank_threshold") {
                config.non_blank_threshold = non_blank_threshold.as_u64().unwrap();
            }
            if let Some(same_screen_threshold) = c.get("same_screen_threshold") {
                config.same_screen_ratio = same_screen_threshold.as_f64().unwrap() as f32;
            }
        }
        config
    }
}

struct Screenshot {
    timestamp: DateTime<Utc>,
    monitor: i64,
    image: RgbaImage,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
struct ScreenEvent {
    t: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,

    image_id: String,
}

#[derive(Debug, serde::Serialize)]
struct Out {
    ch: String,
    data: OutData,
}

#[derive(Debug, serde::Serialize)]
struct OutData {
    kind: String,
    value: ScreenEvent,
}

struct ScreenAgent {
    config: AgentConfig,
    last_image: Option<GrayImage>,
    last_image_id: Option<String>,
}

impl ScreenAgent {
    fn new(config: AgentConfig) -> Self {
        Self {
            config,
            last_image: None,
            last_image_id: None,
        }
    }

    async fn run(&mut self) -> Result<()> {
        let mut interval = time::interval(time::Duration::from_secs(self.config.interval));
        let mut last_interval_period = self.config.interval;

        let mut reader = BufReader::new(stdin());
        let mut line = String::new();

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    self.execute_task().await.unwrap_or_else(|e| log::error!("Error: {}", e));
                    if last_interval_period != self.config.interval {
                        interval = time::interval(time::Duration::from_secs(self.config.interval));
                        last_interval_period = self.config.interval;
                    }
                }
                _ = reader.read_line(&mut line) => {
                    self.process_line(&line).await.unwrap_or_else(|e| log::error!("Error: {}", e));
                    line.clear();
                }
                _ = ctrl_c() => {
                    log::info!("\nShutting down {}.", AGENT_NAME);
                    break;
                }
            }
        }
        Ok(())
    }

    async fn execute_task(&mut self) -> Result<()> {
        let screenshot = self.take_screenshot().await?;
        if screenshot.is_none() {
            return Ok(());
        }
        let screenshot = screenshot.unwrap();

        let start = Instant::now();
        let same = self.is_same(&screenshot);
        let elapsed = start.elapsed();
        log::debug!("is_same elapsed: {:?}", elapsed);

        if same {
            log::debug!("Close to last screenshot");

            let ts = screenshot.timestamp;
            let screen_event = ScreenEvent {
                t: ts.timestamp_millis(),
                image: None,
                image_id: self.last_image_id.clone().unwrap(),
            };
            let out_data = Out {
                ch: KIND.to_string(),
                data: OutData {
                    kind: KIND.to_string(),
                    value: screen_event,
                },
            };
            println!(".OUT {}", serde_json::to_string(&out_data)?);

            return Ok(());
        }

        // convert screenshot image into base64 string

        let ts = screenshot.timestamp;
        let ymd = ts.format("%Y%m%d").to_string();
        let hms = ts.format("%H%M%S").to_string();
        let image = rgba_to_base64_png(&screenshot.image)?;
        let image_id = format!("{}-{}-{}", ymd, hms, screenshot.monitor);

        let screen_event = ScreenEvent {
            t: ts.timestamp_millis(),
            image: Some(image),
            image_id: image_id.clone(),
        };
        let out_data = Out {
            ch: KIND.to_string(),
            data: OutData {
                kind: KIND.to_string(),
                value: screen_event,
            },
        };
        println!(".OUT {}", serde_json::to_string(&out_data)?);

        self.last_image_id = Some(image_id);

        Ok(())
    }

    async fn take_screenshot(&self) -> Result<Option<Screenshot>> {
        log::debug!("take screenshot");
        let monitors = Monitor::all()?;

        for monitor in monitors {
            if monitor.is_primary() {
                // save only the primary monitor
                let screenshot = Screenshot {
                    timestamp: chrono::Utc::now(),
                    monitor: monitor.id() as i64,
                    image: monitor.capture_image()?,
                };
                if self.is_blank(&screenshot.image) {
                    log::debug!("Blank screen: monitor: {}", screenshot.monitor);
                    return Ok(None);
                }
                return Ok(Some(screenshot));
            }
        }
        Ok(None)
    }

    async fn process_line(&mut self, line: &str) -> Result<()> {
        log::debug!("process_line: {}", line);
        if let Some((cmd, args)) = parse_line(line) {
            match cmd {
                ".CONFIG" => {
                    let config = AgentConfig::from(args);
                    log::info!("Update config: {:?}", config);
                    self.config = config;
                }
                ".QUIT" => {
                    log::info!("Quit {}.", AGENT_NAME);
                    std::process::exit(0);
                }
                _ => {
                    log::error!("Unknown command: {}", cmd);
                }
            }
        }
        Ok(())
    }

    fn is_blank(&self, image: &RgbaImage) -> bool {
        let mut count = 0;
        for pixel in image.pixels().step_by(120) {
            if pixel.0[0] >= self.config.almost_black_threshold as u8
                || pixel.0[1] >= self.config.almost_black_threshold as u8
                || pixel.0[2] >= self.config.almost_black_threshold as u8
            {
                count += 1;
            }
            if count >= self.config.non_blank_threshold {
                return false;
            }
        }
        true
    }

    fn is_same(&mut self, screenshot: &Screenshot) -> bool {
        let gray_image = fast_downsample(&screenshot.image, 4);
        if let Some(last_image) = &self.last_image {
            let diff_ratio = get_difference_ratio2(&gray_image, last_image);
            log::debug!("diff_ratio: {}", diff_ratio);
            if diff_ratio < self.config.same_screen_ratio {
                true
            } else {
                self.last_image = Some(gray_image);
                false
            }
        } else {
            self.last_image = Some(gray_image);
            false
        }
    }
}

fn parse_line(line: &str) -> Option<(&str, &str)> {
    if line.is_empty() {
        return None;
    }

    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    if let Some((cmd, args)) = line.split_once(" ") {
        Some((cmd, args))
    } else {
        Some((line, ""))
    }
}

fn rgba_to_base64_png(img: &RgbaImage) -> Result<String> {
    let mut buffer = Cursor::new(Vec::new());
    img.write_to(&mut buffer, ImageFormat::Png)?;
    Ok(base64::engine::general_purpose::STANDARD.encode(buffer.into_inner()))
}

fn fast_downsample(img: &RgbaImage, scale: u32) -> GrayImage {
    let new_width = img.width() / scale;
    let new_height = img.height() / scale;
    let scale_squared = (scale * scale) as u32;

    let mut result = GrayImage::new(new_width, new_height);

    for y in 0..new_height {
        for x in 0..new_width {
            let mut sum = 0u32;

            for dy in 0..scale {
                for dx in 0..scale {
                    let px = img.get_pixel(x * scale + dx, y * scale + dy);
                    // RGBA to Grayscale
                    sum += (px[0] as u32 * 299 + px[1] as u32 * 587 + px[2] as u32 * 114) / 1000;
                }
            }
            result.put_pixel(x, y, image::Luma([(sum / scale_squared) as u8]));
        }
    }

    result
}

fn get_difference_ratio2(img1: &GrayImage, img2: &GrayImage) -> f32 {
    if img1.dimensions() != img2.dimensions() {
        return 1.0;
    }
    let different_pixels = img1
        .pixels()
        .zip(img2.pixels())
        .filter(|(p1, p2)| {
            let diff = if p1.0[0] > p2.0[0] {
                p1.0[0] - p2.0[0]
            } else {
                p2.0[0] - p1.0[0]
            };
            diff > 5 // TODO: setting
        })
        .count();
    different_pixels as f32 / (img1.width() * img1.height()) as f32
}

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short = 'c', long = "config", help = "JSON config string")]
    config: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();
    let config = args.config.as_deref().unwrap_or_default().into();

    log::info!("Starting {}.", AGENT_NAME);

    let mut agent = ScreenAgent::new(config);
    agent.run().await?;

    Ok(())
}
