import { isAdmin } from '$lib/server/auth';
import { error, text } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { getDesign, setPublished } from '$lib/server/db';

export const POST: RequestHandler = async ({ cookies, params }) => {
	if (!isAdmin(cookies)) {
		error(403, 'Unauthorized');
	}

	const design = await getDesign(params.design_id);

	if (!design) {
		error(404, 'Not found');
	}

	if (!design.published) {
		error(400, 'Design not published');
	}

	await setPublished(params.design_id, false);

	return text('Design published');
};
