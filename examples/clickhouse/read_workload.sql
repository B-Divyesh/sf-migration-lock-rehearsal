SELECT count() FROM customers WHERE joined_at > now() - INTERVAL 30 DAY;
