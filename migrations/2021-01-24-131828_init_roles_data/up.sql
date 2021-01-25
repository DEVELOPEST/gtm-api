-- Your SQL goes here
INSERT INTO roles(id, name)
  VALUES (1, 'USER'),
         (2, 'LECTURER'),
         (3, 'ADMIN');

INSERT INTO users(id, email, password) -- password is password
  VALUES (1, 'admin@admin', '$rscrypt$0$CggB$qEh9DpkdcMWP+FIqUHBvug==$T+0YoFbP8a2SSD76frRC/Xlyc8dg3hS+NqnH8tz5y28=$');

INSERT INTO user_role_members("user", role)
  VALUES (1, 3);