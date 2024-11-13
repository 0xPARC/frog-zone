This is a [Next.js](https://nextjs.org) project bootstrapped with [`create-next-app`](https://nextjs.org/docs/app/api-reference/cli/create-next-app).

## Getting Started
```bash
npm install
```

Generate Prisma db Client:
```bash
npx prisma generate
```

Run any db migrations (changes to prisma/schema.prisma):
```bash
npx prisma migrate dev
```

make sure the DATABASE_URL is set correctly in your .env file

First, run the development server:

```bash
npm run dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

To open the Prisma studio:
```bash
npx prisma studio
```

## Setting up your own DB for development

To use your own db for development purposes:

1. Set up a postgreSQL db of you own. For example you can go to supabase and create a new project https://supabase.com/
Click "Connect" to get the url values, add them to the .env like so:

```bash
DATABASE_URL=postgresql://postgres.[address]:[password]@aws-0-ap-southeast-1.pooler.supabase.com:6543/postgres?pgbouncer=true
DIRECT_URL=postgresql://postgres.[address]:[password]@aws-0-ap-southeast-1.pooler.supabase.com:5432/postgres
```
2. Push the current schema
```bash
npx prisma db push
```

3. You are ready to start the server

## Learn More

To learn more about Next.js, take a look at the following resources:

- [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API.
- [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial.

You can check out [the Next.js GitHub repository](https://github.com/vercel/next.js) - your feedback and contributions are welcome!

## Deploy on Vercel

The easiest way to deploy your Next.js app is to use the [Vercel Platform](https://vercel.com/new?utm_medium=default-template&filter=next.js&utm_source=create-next-app&utm_campaign=create-next-app-readme) from the creators of Next.js.

Check out our [Next.js deployment documentation](https://nextjs.org/docs/app/building-your-application/deploying) for more details.
