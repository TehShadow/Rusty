📚 API Routes Documentation
🔐 Authentication
Method	Route	Body (JSON)	Description
POST	/register	{ "username": "nova", "password": "..." }	Register a new user
POST	/login	{ "username": "nova", "password": "..." }	Log in, receive JWT + session cookie
GET	/me	Bearer token required	Get current user info from JWT

👤 User Info
Method	Route	Description
GET	/users/:id	Get user info by UUID

💬 Direct Messages (DM)
Method	Route	Body (JSON)	Description
POST	/dm/:user_id	{ "content": "Hey there!" }	Send DM to a specific user
GET	/dm/:user_id	(none)	Load DMs between two users

🏠 Rooms (Servers)
Method	Route	Body (JSON)	Description
POST	/rooms	{ "name": "Cool Room" }	Create a new room
GET	/rooms	(none)	List rooms user has joined
POST	/rooms/:id/join	(none)	Join a room by ID
POST	/rooms/:id/messages	{ "content": "Hello everyone!" }	Send message in a room
GET	/rooms/:id/messages	(none)	Load all messages in room

👥 User Relationships (Friends, Blocks)
Method	Route	Description
POST	/relationships/:id	Send friend request
POST	/relationships/:id/accept	Accept friend request
POST	/relationships/:id/block	Block a user
DELETE	/relationships/:id	Remove friend or cancel/block
GET	/relationships/friends	List current friends
GET	/relationships/pending	List pending requests

🛠 Dev/Testing (Optional)
Method	Route	Description
GET	/health	Basic health check
POST	/dev/flush-db	Reset DB (dev only)

🧪 Auth Required
All routes except /register, /login, and /health require:

✅ Valid JWT in Authorization: Bearer <token>

✅ Or a valid session cookie (if implemented)

