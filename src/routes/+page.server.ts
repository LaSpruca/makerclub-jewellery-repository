import type { PageServerLoad } from './$types';
import { getUnpublished } from '$lib/server/db';
import { createClient } from '@vercel/postgres';

export const load: PageServerLoad = async () => {
	return {
		initial_items: await getUnpublished()
	};
};
