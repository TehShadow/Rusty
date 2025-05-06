'use client';

import { useUser } from '@/hooks/useUser';

export default function ProfilePage() {
  const { user, loading } = useUser();

  if (loading) return <p>Loading...</p>;
  if (!user) return <p>User not found</p>;

  return (
    <div>
      <h1>Welcome, {user.username}!</h1>
      <p>ID: {user.id}</p>
    </div>
  );
}
