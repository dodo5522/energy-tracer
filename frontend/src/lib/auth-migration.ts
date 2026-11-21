import { betterAuth } from 'better-auth';
import { Pool } from 'pg';

export const auth = betterAuth({
  database: new Pool({
    host: process.env?.DB_HOST,
    port: parseInt(process.env?.DB_PORT ?? '5432', 10),
    database: process.env?.DB_NAME,
    user: process.env?.DB_MIGRATOR_NAME,
    password: process.env?.DB_MIGRATOR_PASSWORD,
    options: "-c search_path=auth",
  }),
  emailAndPassword: {
    enabled: true,
  },
});
