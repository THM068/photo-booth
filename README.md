Migrate Database run
DATABASE_URL="postgres://postgres:postgres@localhost:21852/photo-booth" sea-orm-cli migrate refresh 

generate entities
sea-orm-cli generate entity -u postgres://postgres:postgres@localhost:21852/photo-booth -o src/entities
