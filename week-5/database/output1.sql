-- select * from product;

-- select name,product_id from product;

-- select * from product where product_id = 7;

-- select count(*) as total_products from product;

----------------------------------------------------------------------chapter 2---------------------------------------------------------------------------------------

-- select name from product where price>20;

-- select name from product where manufacturer='Mad Inventors Inc.';

-- select name,price from product ORDER BY price asc;

-- select name from product where price>20 and manufacturer='Silly Supplies Co.';

-- select name from product where price>20 and not manufacturer='Silly Supplies Co.';

----------------------------------------------------------------------chapter 3---------------------------------------------------------------------------------------

create table employees(emp_id primary key,name,department,salary,joining_date) ;

insert into employees values(1,'John Doe','Sales',50000,'2020-01-15'),
(2,'Jane Smith','Marketing',60000,'2019-03-22'),
(3,'Mike Johnson','IT',75000,'2021-07-10'),
(4,'Emily Davis','HR',55000,'2018-11-05'),
(5,'David Wilson','Finance',70000,'2020-06-30');

ALTER table employees add column email varchar;

drop table employees;

select * from employees;


