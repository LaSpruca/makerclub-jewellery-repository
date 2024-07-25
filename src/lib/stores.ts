import { writable } from 'svelte/store';
import type { UploadRow, UserRow } from './server/db';

export const items = writable<(UploadRow & UserRow)[]>();
