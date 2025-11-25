-- Seeder for table todos
INSERT INTO todos (id, title, completed, created_at) VALUES (1, 'Buy groceries', false, '2025-11-24 15:59:03.627904 +00:00') ON CONFLICT (id) DO UPDATE SET title = EXCLUDED.title, completed = EXCLUDED.completed, created_at = EXCLUDED.created_at;
INSERT INTO todos (id, title, completed, created_at) VALUES (2, 'Walk the dog', true, '2025-11-24 15:59:03.627904 +00:00') ON CONFLICT (id) DO UPDATE SET title = EXCLUDED.title, completed = EXCLUDED.completed, created_at = EXCLUDED.created_at;
INSERT INTO todos (id, title, completed, created_at) VALUES (3, 'Learn Rust', false, '2025-11-24 15:59:03.627904 +00:00') ON CONFLICT (id) DO UPDATE SET title = EXCLUDED.title, completed = EXCLUDED.completed, created_at = EXCLUDED.created_at;
INSERT INTO todos (id, title, completed, created_at) VALUES (4, 'test', false, '2025-11-24 16:03:32.550053 +00:00') ON CONFLICT (id) DO UPDATE SET title = EXCLUDED.title, completed = EXCLUDED.completed, created_at = EXCLUDED.created_at;
