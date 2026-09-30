export interface User {
  id: string;
  username: string;
  email: string;
}

let user = $state<User | null>(null);

export function setUser(data: User) {
  user = data;
}

export function getUser() {
  return user;
}
