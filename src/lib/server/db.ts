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
export async function getItems(offset?: number) {
	if (offset) {
		return (
			await client.sql<UserRow & UploadRow>`SELECT *
FROM uploads 
LEFT JOIN users ON uploads.userid = users.userid
ORDER BY title desc
LIMIT 20
OFFSET ${offset * 20}`
		).rows;
	} else {
		return (
			await client.sql<UserRow & UploadRow>`SELECT *
FROM uploads 
LEFT JOIN users ON uploads.userid = users.userid
ORDER BY title desc
LIMIT 20`
		).rows;
	}
}

/**
 * Get a design
 *
 * @param id The ID of the design
 */
export async function getDesign(id: string) {
	return (await client.sql<UploadRow>`SELECT * FROM uploads WHERE id = ${id} LIMIT 1`).rows[0];
}

/**
 * Publish or unpublish a design
 *
 * @param id The id of the design
 */
export async function setPublished(id: string, published: boolean) {
	await client.sql`UPDATE uploads SET published = ${published} where ID = ${id}`;
}
