<h1 style="display: flex; justify-content: center; gap: 10px; flex-direction: row;">
<svg style="width:30px" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" x="-20px" y="0px" viewBox="8 0 80 100" enable-background="new 0 0 100 100" xml:space="preserve"><path fill="currentColor" d="M69.657,87.453c-22.385,4.014-39.425-0.06-39.425-0.06l0.812,5.461c0,0,16.689,3.856,37.993,0.113  L69.657,87.453z"></path><path fill="currentColor" d="M63.02,9.855C52.068,1.266,41.726,7.227,38.312,9.693c-11.913,0.902-19.366,2.909-19.366,2.909l1.275,8.48  c0.183-0.049,26.843-7.177,59.448,0l1.275-8.481C74.717,11.231,68.7,10.362,63.02,9.855z M21.898,18.663l-0.707-4.15  c0,0,3.798-1.153,9.828-1.686l0.606,4.194C25.704,17.516,21.945,18.649,21.898,18.663z"></path><path fill="currentColor" d="M21.317,25.008l8.741,58.156c0,0,17.216,4.12,39.773,0l8.741-58.155  C47.17,18.096,21.494,24.961,21.317,25.008z M36.267,73.456l-5.904-38.401c1.357-0.19,4.228-0.55,8.227-0.837l1.632,39.648  C38.927,73.755,37.608,73.456,36.267,73.456z M52.335,74.663c-1.431,0.019-2.948,0.012-4.531-0.026l-1.511-42.45  c2.298-0.059,4.778-0.074,7.406-0.022L52.335,74.663z M63.281,73.668c-1.061,0.115-2.383,0.238-3.928,0.347l1.642-39.885  c2.659,0.168,5.419,0.409,8.252,0.739L63.281,73.668z"></path></svg>
deleterr
</h1>

## Overview

deleterr is an application that integrates Radarr, Overseerr, Sonarr, and Tautulli (ROST) to automatically delete media once it has been watched by the requesting user.

- **Backend:** Rust
- **Frontend:** VueJS, Typescript, Tailwind

## Getting Started

### Prerequisites

When you build using cargo it may complain about missing some packages. Install them.

### Distributon ###

This is very early so you would be running pre-alpha and incomplete software. See below on how to install. In the future, I expect to provide a binary and a docker image.

### Installation

**Only for development (or running pre-alpha)**

1. Clone the repository:

   ```bash
   git clone https://github.com/omarmir/deleterr.git
   cd deleterr
   
2. Build the binary
    ```bash
    cargo build
    cargo run

3. Build the frontend
    ```bash
    yarn install
    yarn build
    
### Contributing ###
Contributions are welcome! As the project is in its early stages and likely niche, feel free to open issues, propose new features, or submit pull requests. For major changes, please open an issue first to discuss the changes.

### Features ###

**To reach MVP**

* [x] View requests with available media
* [x] Display if the requesting user has watched the media
* [x] Mark or unmark a media as excluded from automatic deletion
* [x] Delete movies
* [ ] Allow marking specific users that are exempted from automatic deletion
* [ ] Setup credentials and authentication requirement
* [ ] Use persistent store for ROST instead of .env file
* [ ] **Setup schedule for automatic deletion**
* [ ] Mobile friendly

**Additional features planned**
* [ ] Allow for time based deletion
* [ ] Handle TV shows in Sonarr
* [ ] Notification to the users when their file has been removed
* [ ] Notification to the user when their file is about to be deleted due to time based limits
* [ ] Allow user to request more time before file is removed - customizable

### Requirements ###
This application requires access to the Radarr, Sonarr, Overseerr, and Tautulli APIs.

### License ###
This project is licensed under the MIT License.
