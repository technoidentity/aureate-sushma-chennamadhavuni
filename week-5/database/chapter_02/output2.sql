-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- PART1-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- ---- -- -- -- -- -- 

select * from product;

select * from review;   

select * from review inner join product on review.product_id=product.product_id where review_id = 6;
 
select count(distinct(product_id)) from review;

-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- PART2-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- ---- -- -- -- -- -- 

select name,price,review_id,review_text from product join review on product.product_id=review.product_id;

select name,review_id,review_text from review inner join product on product.product_id=review.product_id where name='Selfie Toaster';

select name,review_text from review join product on product.product_id=review.product_id where review_text like '%great%';

select manufacturer,name,review_id,review_text from review join product on review.product_id=product.product_id where manufacturer='Mad Inventors Inc.';

-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- PART3-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- ---- -- -- -- -- -- 


select name,review_id,review_text from product left join  review on product.product_id=review.product_id;

select product.product_id,name,review_text from product left join review ON product.product_id=review.product_id where review_text is null;

select product.product_id,name,count(review_id) as review_count from product 
left join review on product.product_id=review.product_id 
group by product.product_id,name order by review_count desc;

-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- PART4-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- ---- -- -- -- -- -- 

select  name,review_id,review_text,datetime from review inner JOIN product on review.product_id=product.product_id where datetime > '2022-01-05 00:00:00';

select name,review_text,datetime from review join  product on review.product_id=product.product_id where review_text like '%disappointing%'

select product.product_id,name,price,count(review_id) as review_count from product join review on product.product_id=review.product_id 
WHERE CAST(price AS REAL) > 20
group by product.product_id, name, price
HAVING COUNT(review.review_id) >= 1;

select name,count(review_id) as ct from product left join review on  product.product_id=review.product_id group by name;

select name,count(review_id) as ct from product left join review ON product.product_id=review.product_id group by name having ct > 2

-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- PART5-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- ---- -- -- -- -- -- 

select name,count(review_id) as review_count from product left join review on product.product_id=review.product_id group by product.product_id,name having review_count > 2 order by review_count desc limit 1;

select manufacturer,count(review_id) as review_count from product left join review on product.product_id=review.product_id group by manufacturer;

