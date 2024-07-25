import { ADMIN_PASSWORD } from '$env/static/private';

export function isAdmin(cookies: Cookies) {
	const adminPassword = cookies.get('admin_password');
	return adminPassword && adminPassword == ADMIN_PASSWORD;
}
