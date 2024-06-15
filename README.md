# newswave
A simple newsletter app using Rust

## Routes
- /api/health_check - external monitoring services
- /api/subscribe - new user can subscribe to the newsletter
- /api/subscribe/:token - new user verifies their subcription
- /api/publish - admins to publish new newsletters

 Uses Postgres With Redis for user storage and session management
 Simple React Frontend for the admins to write new newletter
