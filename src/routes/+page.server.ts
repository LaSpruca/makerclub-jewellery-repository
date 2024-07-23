import { createPool } from '@vercel/postgres';
import type { PageServerLoad } from './$types';
import { POSTGRES_URL } from '$env/static/private';

const client = createPool({ connectionString: POSTGRES_URL });
export const load: PageServerLoad = async () => {
	return {
		all_items:
			await client.sql`SELECT * FROM uploads LEFT JOIN users u on uploads.userid = u.userid`
	};
};
