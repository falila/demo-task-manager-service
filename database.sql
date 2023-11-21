drop table if exists task_list;
drop table if exists task_item;

CREATE TABLE task_list (
	id serial PRIMARY KEY,
	title VARCHAR ( 255 ) NOT NULL
);


CREATE TABLE task_item (
	id serial PRIMARY KEY,
	title  VARCHAR ( 255 ) NOT NULL,
	checked BOOLEAN NOT NULL DEFAULT FALSE,
    list_id INTEGER NOT NULL,
    FOREIGN KEY (list_id) REFERENCES task_list(id)
);


insert into task_list (title) values ('Task 1'), ('Task 2');
insert into task_item (title, list_id) values ('Item 1', 1),('Item 2', 1),('Item 12', 2);