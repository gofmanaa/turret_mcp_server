#![allow(dead_code)]
use rmcp::{
    Error as McpError, ServerHandler,
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::*,
    schemars, tool, tool_handler, tool_router,
};
use serialport::SerialPort;
use std::io::Write;
use std::sync::{Arc, Mutex};

/*
Serial port commands:
"X90Y90/n" - move to center
"FIRE\n" - fire
 */
#[derive(Debug, serde::Deserialize, schemars::JsonSchema, Clone)]
pub struct ServoPos {
    #[schemars(description = "Turn the Turret left or right X range:0..180, center:90")]
    pub x: i32,
    #[schemars(description = "Turn the Turret up or down Y range:0..180, center:90")]
    pub y: i32,
}

#[derive(Clone)]
pub struct Turret {
    serial_port: Arc<Mutex<Box<dyn SerialPort>>>,
    tool_router: ToolRouter<Turret>,

    fire_attempt: Arc<Mutex<u8>>,
}

fn setup_serial_port() -> Box<dyn SerialPort> {
    // uncomment to see all available ports
    // let available_ports = serialport::available_ports().expect("No serial ports found!");
    // for port in available_ports {
    //     println!("Available port: {}, type: {:?}", port.port_name, port.port_type);
    // }
    let port_name = "/dev/ttyUSB0";
    let baud_rate = 9600;

    match serialport::new(port_name, baud_rate).open() {
        Ok(p) => p,
        Err(e) => panic!("Failed to open \"{port_name}\". Error: {e}"),
    }
}

#[tool_router]
impl Turret {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            serial_port: Arc::new(Mutex::new(setup_serial_port())),
            tool_router: Self::tool_router(),
            fire_attempt: Arc::new(Mutex::new(6u8)), // 6 bullets
        }
    }

    fn _create_resource_text(&self, uri: &str, name: &str) -> Resource {
        RawResource::new(uri, name.to_string()).no_annotation()
    }

    #[tool(description = "Get the current bullets value")]
    async fn get_bullets(&self) -> Result<CallToolResult, McpError> {
        let counter = self.fire_attempt.lock().unwrap();
        Ok(CallToolResult::success(vec![Content::text(
            counter.to_string(),
        )]))
    }

    #[tool(description = "Repeat what you say")]
    fn echo(&self, Parameters(object): Parameters<JsonObject>) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::Value::Object(object).to_string(),
        )]))
    }

    #[tool(description = "Turret fire")]
    async fn fire(&self) -> Result<CallToolResult, McpError> {
        let mut port = self.serial_port.lock().unwrap();
        let command = "FIRE\n";
        tracing::debug!("Attempting to send command to serial port: {}", command);
        let write_result = port.write_all(command.as_bytes());
        match write_result {
            Ok(_) => {
                tracing::debug!("Successfully wrote command to serial port.");
                let mut bullets = self.fire_attempt.lock().unwrap();
                if *bullets == 0 {
                    return Ok(CallToolResult::success(vec![Content::text(format!(
                        "Turret can't do {}, bullet left: {}",
                        command, *bullets
                    ))]));
                }
                *bullets -= 1;

                Ok(CallToolResult::success(vec![Content::text(format!(
                    "Turret did {}, bullet left: {}",
                    command, *bullets
                ))]))
            }
            Err(e) => {
                tracing::error!("Failed to write to serial port: {}", e);
                Err(McpError::new(
                    rmcp::model::ErrorCode(-32000),
                    e.to_string(),
                    None,
                ))
            }
        }
    }

    #[tool(description = "Turn the Turret left or right X 0..180 and up or down Y range:0..180")]
    async fn set_turret_position(
        &self,
        Parameters(ServoPos { x, y }): Parameters<ServoPos>,
    ) -> Result<CallToolResult, McpError> {
        let mut port = self.serial_port.lock().unwrap();
        let command = format!("X{}Y{}\n", x as u8, y as u8);
        tracing::debug!(
            "Attempting to send command to serial port: {:?}",
            command.as_bytes()
        );
        let write_result = port.write_all(command.as_bytes());
        match write_result {
            Ok(_) => {
                tracing::debug!("Successfully wrote command to serial port.");
                Ok(CallToolResult::success(vec![Content::text(format!(
                    "Turned turret, {command}"
                ))]))
            }
            Err(e) => {
                tracing::error!("Failed to write to serial port: {}", e);
                Err(McpError::new(
                    rmcp::model::ErrorCode(-32000),
                    e.to_string(),
                    None,
                ))
            }
        }
    }
}

#[tool_handler]
impl ServerHandler for Turret {}
