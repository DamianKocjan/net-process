# net-process

A simple, lightweight desktop utility to visualize netstat data and monitor active process connections in a graphical interface.
Features

- Network Statistics: View real-time Interface, IPv4, IPv6, ICMP, and TCP statistics.
- Connection Tracking: Monitor active TCP/UDP connections mapped to specific processes (e.g., Discord, Chrome).
- Process Filtering: Easily identify which local addresses are communicating with foreign endpoints.

## Screenshots

<img width="800" height="632" alt="image" src="https://github.com/user-attachments/assets/f591d0f1-bcd2-4ff9-932f-b0142c7d4d9e" />
<img width="805" height="630" alt="image" src="https://github.com/user-attachments/assets/351fbf34-e458-483d-8eb6-00d24d391333" />

## Tech Stack

- Rust
- Tauri
- ReactJS

## Installation

You don't need to build the project from source to use it. You can download the latest version for your operating system directly from the Releases page:

- Navigate to the Releases section.
- Download the installer or executable for your platform.
- Run the application.

## Running locally

### Requirements

- NodeJS
- Rust

Clone the repository:

```bash
git clone https://github.com/DamianKocjan/net-process.git
```

Install dependencies:

```Bash
npm install
```

Run in development mode:
```bash
npm run tauri dev
```
