import { createPool } from '@vercel/postgres';
import { POSTGRES_URL } from '$env/static/private';

const client = createPool({ connectionString: POSTGRES_URL });

export type UploadRow = {
	id: string;
	userid: number;
	title: string;
	description: string;
	svg_url: string;
	thumbnail_url: string;
	published: boolean;
};

export type UserRow = {
	userid: number;
	username: string;
	nickname?: string;
	avatar_url: string;
};

/**
 * Get 20 of the jewellery file that have been published
 *
 * @param offset The number of pages to skip
 */
export async function getPublished(offset?: number) {
	if (offset) {
		return (
			await client.sql<UserRow & UploadRow>`SELECT *
FROM uploads
LEFT JOIN users ON uploads.userid = users.userid
WHERE published
ORDER BY title desc
LIMIT 20
OFFSET ${offset * 20}`
		).rows;
	} else {
		return (
			await client.sql<UserRow & UploadRow>`SELECT *
FROM uploads
LEFT JOIN users ON uploads.userid = users.userid
WHERE published
ORDER BY title desc
LIMIT 20`
		).rows;
	}
}

/**
 * Get 20 of the jewellery items await aporval
 *
 * @param offset The number of pages to skip
 */
export async function getUnpublished(offset?: number) {
	if (offset) {
		return (
			await client.sql<UserRow & UploadRow>`SELECT *
FROM uploads 
LEFT JOIN users ON uploads.userid = users.userid
WHERE not published
ORDER BY title desc
LIMIT 20
OFFSET ${offset * 20}`
		).rows;
	} else {
		return (
			await client.sql<UserRow & UploadRow>`SELECT *
FROM uploads 
LEFT JOIN users ON uploads.userid = users.userid
WHERE not published
ORDER BY title desc
LIMIT 20`
		).rows;
	}
}
