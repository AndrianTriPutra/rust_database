update books 
set title = 'a123',
	author = 'b123',
	quantity = 13 
where uuid = 'abc'
RETURNING *