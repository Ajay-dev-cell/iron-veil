# IronVeil Dashboard

The web dashboard for IronVeil database proxy. Built with Next.js, Tailwind CSS, and Shadcn UI.

## Features

- **Dashboard**: Real-time system status, active connections, and rules overview
- **Masking Rules**: View, add, and manage data masking rules
- **PII Scanner**: Scan database for potential PII columns with one-click rule creation
- **Live Inspector**: Real-time query monitoring with masked data details
- **Settings**: Global masking controls, system status, and configuration export

## Getting Started

### Prerequisites

- Node.js 18+
- The IronVeil proxy running on port 3001 (API)

### Development

```bash
# Install dependencies
npm install

# Run development server
npm run dev
```

Open [http://localhost:3000](http://localhost:3000) to view the dashboard.

### Production Build

```bash
npm run build
npm start
```

## Tech Stack

- **Framework**: Next.js 16 (App Router)
- **Styling**: Tailwind CSS 4
- **Components**: Shadcn UI
- **State Management**: TanStack Query (React Query)
- **Icons**: Lucide React

## Project Structure

```
web/
├── public/
│   └── logo.png           # IronVeil logo
├── src/
│   ├── app/
│   │   ├── layout.tsx     # Root layout with sidebar
│   │   ├── page.tsx       # Dashboard
│   │   ├── globals.css    # Global styles
│   │   ├── inspector/     # Live query inspector
│   │   ├── rules/         # Masking rules management
│   │   ├── scan/          # PII scanner
│   │   └── settings/      # Global settings
│   ├── components/
│   │   ├── sidebar.tsx    # Navigation sidebar
│   │   ├── providers.tsx  # React Query provider
│   │   └── ui/            # Shadcn UI components
│   └── lib/
│       └── utils.ts       # Utility functions
├── package.json
└── next.config.ts
```

## API Endpoints

The dashboard connects to the IronVeil Management API:

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Service health check |
| `/rules` | GET | List all masking rules |
| `/rules` | POST | Add a new masking rule |
| `/config` | GET | Get current configuration |
| `/config` | POST | Update configuration |
| `/connections` | GET | Get active connection count |
| `/logs` | GET | Get recent query logs |
| `/scan` | POST | Trigger PII scan |

## Development

```bash
# Run linter
npm run lint

# Type check
npx tsc --noEmit
```
