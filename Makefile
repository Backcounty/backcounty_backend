ifneq (,$(wildcard .env))
    include .env
    export
endif

migration_dir:=./migration
generate:
	 sea-orm-cli migrate generate --migration-dir ${migration_dir} --database-url $(DATABASE_URL) $(MIGRATION_NAME)
