import { writable } from 'svelte/store';

const ROLES = {
  ADMIN: 'admin',
  CASHIER: 'cashier',
  SUPERVISOR: 'supervisor',
} as const;

type Role = (typeof ROLES)[keyof typeof ROLES];

export interface UserEntity {
  id: string;
  email: string;
  name: string;
  role: Role;
  avatar: string;
}

export const userStore = writable<UserEntity | null>(null);
