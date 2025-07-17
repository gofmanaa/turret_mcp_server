# Turret Controller MCP Server

## Description

This project is an MCP (Model Context Protocol) server that controls a turret using serial communication. It exposes tools to:

![turret.png](./turret.png)


*   Fire the turret
*   Set the turret's position (pan and tilt)
*   Get the current number of bullets

## Usage

This server exposes tools that can be accessed via an MCP client.

### Tools

*   **fire**: Fires the turret.
*   **set_turret_position**: Sets the turret's position. Requires `x` and `y` coordinates (0-180).
*   **get_bullets**: Gets the current number of bullets.
*   **echo**: Repeat what you say.

## Installation

1.  Make sure you have Rust installed. If not, install it from [https://www.rust-lang.org/](https://www.rust-lang.org/).
2.  Clone the repository.
3.  Navigate to the project directory.
4.  Run `cargo build -r` to build the project.
5.  MCP Agent Plugin node require settings.json like this to command or URL to connect MCP servers
```json
{
  "mcpServers": {
    "turret-mcp-server": {
      "command": "/path/to/turret_mcp_server/target/release/turret_mcp_server",
      "alwaysAllow": [
        "set_turret_position",
        "get_bullets",
        "fire"
      ]
    }
  }
}
```
6. Connect the turret to USB

## How to use
In chat context you can simple write: 
`turn the turret left, then right and shoot`



### Serial Port Configuration

The server uses the `/dev/ttyUSB0` serial port by default. You may need to modify the [turret_mcp_server.rs](src/turret_mcp_server.rs) file to use the correct serial port for your system.

### Dependencies

The project uses the following dependencies:

*   `anyhow`
*   `rmcp`
*   `serialport`
*   `serde_json`
*   `tracing_subscriber`

These dependencies are managed by Cargo, the Rust package manager.

## Docker

You can also build and run this project using Docker.

### Building the Docker Image

To build the Docker image, run the following command in the project directory:

`docker build -t turret-mcp-server .`

### Running the Docker Container

To run the Docker container, you need to pass the serial port device from the host to the container. You can do this using the `--device` flag.

`docker run -it --device=/dev/ttyUSB0 turret-mcp-server`

The SSE MCP configuration (settings.json) for Docker looks like this:
```json
{
  "mcpServers": {
    "turret-mcp-server": {
      "url": "http://127.0.0.1:8080/sse"
    }
  }
}
```

**Note:** Make sure to replace `/dev/ttyUSB0` with the correct serial port for your system.
