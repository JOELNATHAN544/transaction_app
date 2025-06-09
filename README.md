# Transaction App

A modern web application for handling online transactions, built with Rust and React.

## Tech Stack

### Backend
- Rust (Actix-web)
- PostgreSQL
- SQLx for database operations
- Stripe for payment processing

### Frontend
- React with TypeScript
- Tailwind CSS
- shadcn/ui for components
- Vite for build tooling

## Project Structure

```
transaction_app/
├── backend/                 # Rust backend
│   ├── src/
│   │   ├── main.rs         # Entry point
│   │   ├── routes/         # API routes
│   │   ├── models/         # Data models
│   │   ├── services/       # Business logic
│   │   └── config/         # Configuration
│   └── Cargo.toml          # Rust dependencies
├── frontend/               # React frontend
│   ├── src/
│   │   ├── components/     # React components
│   │   ├── pages/         # Page components
│   │   ├── services/      # API services
│   │   └── utils/         # Utility functions
│   └── package.json       # Node dependencies
└── docker/                # Docker configuration
```

## Getting Started

### Prerequisites
- Rust (latest stable)
- Node.js (v18 or later)
- PostgreSQL
- Stripe account (for payments)

### Backend Setup
1. Navigate to the backend directory:
   ```bash
   cd backend
   ```
2. Create a `.env` file with your configuration:
   ```
   DATABASE_URL=postgres://postgres:postgres@localhost:5432/transaction_app
   HOST=127.0.0.1
   PORT=8080
   ```
3. Run the backend:
   ```bash
   cargo run
   ```

### Frontend Setup
1. Navigate to the frontend directory:
   ```bash
   cd frontend
   ```
2. Install dependencies:
   ```bash
   npm install
   ```
3. Run the development server:
   ```bash
   npm run dev
   ```

## Features
- User authentication
- Product management
- Secure payment processing
- Transaction history
- Admin dashboard

## Contributing
1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License
This project is licensed under the MIT License - see the LICENSE file for details. 