# deleterr

## Overview

deleterr is an application that integrates Radarr, Overseerr, Sonarr, and Tautulli (ROST) to automatically delete media once it has been watched by the requesting user.

- **Backend:** Rust
- **Frontend:** VueJS, Typescript, Tailwind

## Getting Started

### Prerequisites

When you build using cargo it may complain about missing some packages. Install them.

### Distributon

This is very early so you would be running pre-alpha and incomplete software. See below on how to install. In the future, I expect to provide a binary and a docker image.

### Installation

**Only for development (or running pre-alpha)**

1. Clone the repository:

   ```bash
   git clone https://github.com/omarmir/deleterr.git
   cd deleterr

   ```

2. Build the binary

   ```bash
   cargo build
   cargo run

   ```

3. Build the frontend
   ```bash
   yarn install
   yarn build

   ```

### Contributing

Contributions are welcome! As the project is in its early stages and likely niche, feel free to open issues, propose new features, or submit pull requests. For major changes, please open an issue first to discuss the changes.

### Features

**To reach MVP**

- [x] View requests with available media
- [x] Display if the requesting user has watched the media
- [x] Mark or unmark a media as excluded from automatic deletion
- [x] Delete movies
- [ ] Allow marking specific users that are exempted from automatic deletion
- [ ] Setup credentials and authentication requirement
- [ ] Use persistent store for ROST instead of .env file
- [ ] **Setup schedule for automatic deletion**
- [ ] Mobile friendly

**Additional features planned**

- [ ] Allow for time based deletion
- [ ] Handle TV shows in Sonarr
- [ ] Notification to the users when their file has been removed
- [ ] Notification to the user when their file is about to be deleted due to time based limits
- [ ] Allow user to request more time before file is removed - customizable

### Requirements

This application requires access to the Radarr, Sonarr, Overseerr, and Tautulli APIs.

### License

This project is licensed under the MIT License.
