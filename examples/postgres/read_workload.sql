SELECT count(*) FROM customers WHERE joined_at > now() - interval '30 days';
