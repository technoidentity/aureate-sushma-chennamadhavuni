-- select * from product;

-- select name,product_id from product;

-- select * from product where product_id = 7;

-- select count(*) as total_products from product;

-----------------------------------------------chapter 2---------------------------------------------------------------------------------------

select name from product where price>20;

select name from product where manufacturer='Mad Inventors Inc.';

select name,price from product ORDER BY price asc;

select name from product where price>20 and manufacturer='Silly Supplies Co.';

select name from product where price>20 and not manufacturer='Silly Supplies Co.';


