-- Your SQL goes here
-- 启用外键约束（SQLite 默认关闭）
PRAGMA foreign_keys = ON;

-- 创建关联表
CREATE TABLE
    books_authors (
        book_id INTEGER NOT NULL,
        author_id INTEGER NOT NULL,
        -- 外键约束（级联删除）
        FOREIGN KEY (book_id) REFERENCES books (id) ON DELETE CASCADE,
        FOREIGN KEY (author_id) REFERENCES authors (id) ON DELETE CASCADE,
        -- 复合主键
        PRIMARY KEY (book_id, author_id)
    );

-- 为外键列创建索引（提高关联查询性能）
CREATE INDEX idx_books_authors_book_id ON books_authors (book_id);

CREATE INDEX idx_books_authors_author_id ON books_authors (author_id);

-- 添加唯一约束（如果未包含在主键中）
CREATE UNIQUE INDEX idx_unique_book_author ON books_authors (book_id, author_id);