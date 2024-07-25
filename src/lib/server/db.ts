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
 * Get the jewellery file that have been published
 */
export async function getPublished() {
	return (
		await client.sql<UserRow & UploadRow>`SELECT *
FROM uploads
LEFT JOIN users ON uploads.userid = users.userid
WHERE published
ORDER BY title desc`
	).rows;
}

/**
 * Get the jewellery items await aporval
 */
export async function getItems() {
	return (
		await client.sql<UserRow & UploadRow>`SELECT *
FROM uploads 
LEFT JOIN users ON uploads.userid = users.userid
ORDER BY title desc`
	).rows;
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
 * @param published weather or not the design should be published or unpublished
 */
export async function setPublished(id: string, published: boolean) {
	await client.sql`UPDATE uploads SET published = ${published} where ID = ${id}`;
}

/**
 * Delete a submission
 *
 * @param id The id of submission to delete
 */
export async function deleteSubmission(id: string) {
	await client.sql`DELETE FROM uploads WHERE id = ${id}`;
}
