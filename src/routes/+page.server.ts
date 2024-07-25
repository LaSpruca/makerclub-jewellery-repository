import type { PageServerLoad } from './$types';
import { getItems, getPublished } from '$lib/server/db';
import { isAdmin } from '$lib/server/auth';

export const load: PageServerLoad = async ({ cookies }) => {
	if (isAdmin(cookies)) {
		return {
			is_admin: true,
			initial_items: await getItems()
		};
	}

	return {
		is_admin: false,
		initial_items: await getPublished()
	};
};
