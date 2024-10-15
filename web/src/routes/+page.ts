import type { PageLoad } from './$types';

export const load: PageLoad = async (context) => {
	const result = await fetch('http://localhost:8000/api/users');
	return {
		users: await result.json()
	};
};
