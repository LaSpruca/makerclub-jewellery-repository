import { isAdmin } from '$lib/server/auth';
import { error, text } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { deleteSubmission, getDesign } from '$lib/server/db';

export const POST: RequestHandler = async ({ cookies, params }) => {
	if (!isAdmin(cookies)) {
		error(403, 'Unauthorized');
	}

	const design = await getDesign(params.design_id);

	console.log(design);

	if (!design) {
		error(404, 'Not found');
	}

	if (design.published) {
		error(400, 'Cannot remove a published design');
	}

	await deleteSubmission(params.design_id);

	return text('Design published');
};
