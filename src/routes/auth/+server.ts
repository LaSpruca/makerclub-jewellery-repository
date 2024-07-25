import { error, redirect, text } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { ADMIN_PASSWORD } from '$env/static/private';

export const GET: RequestHandler = async ({ cookies, url }) => {
	const password = url.searchParams.get('password');

	if (!password || password !== ADMIN_PASSWORD) {
		error(403, 'Unauthorized');
	}

	cookies.set('admin_password', ADMIN_PASSWORD, {
		httpOnly: true,
		path: '/',
		// Max age one day
		maxAge: 24 * 60 * 60
	});

	redirect(307, '/');
};
