import { betterAuth } from 'better-auth';
import { Pool } from 'pg';
import { tanstackStartCookies } from 'better-auth/tanstack-start';

export const auth = betterAuth({
  database: new Pool({
    host: process.env?.DB_HOST,
    port: parseInt(process.env?.DB_PORT ?? '5432', 10),
    database: process.env?.DB_NAME,
    user: process.env?.DB_AUTH_OPERATOR_NAME,
    password: process.env?.DB_AUTH_OPERATOR_PASSWORD,
    options: "-c search_path=auth",
  }),
  emailAndPassword: {
    enabled: true,
  },
  plugins: [
    tanstackStartCookies(),
  ],
});
