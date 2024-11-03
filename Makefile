
# Makefile để quản lý việc tạo và cập nhật các thư mục và tệp tin trong thư mục src

# Định nghĩa các thư mục cần tạo trong src
DIRS = src src/data src/events src/unit

# Định nghĩa các tệp tin cần tạo trong src
FILES = \
	src/data/user.rs \
	src/data/item.rs \
	src/unit/store.rs \
	src/unit/work.rs \
	src/unit/handler.rs \
	src/main.rs

# Mục tiêu mặc định
all: dirs files

# Tạo các thư mục
dirs:
	@echo "Creating directories..."
	@for dir in $(DIRS); do \
		mkdir -p $$dir; \
		echo "Created directory: $$dir"; \
	done

# Tạo các tệp tin
files:
	@echo "Creating files..."
	@for file in $(FILES); do \
		touch $$file; \
		echo "Created file: $$file"; \
	done

# Xóa các thư mục và tệp tin
clean:
	@echo "Cleaning up..."
	@for dir in $(DIRS); do \
		rm -rf $$dir; \
		echo "Removed directory: $$dir"; \
	done
	@for file in $(FILES); do \
		rm -f $$file; \
		echo "Removed file: $$file"; \
	done

# Hiển thị cách sử dụng
help:
	@echo "Usage:"
	@echo "  make all     - Create directories and files"
	@echo "  make dirs    - Create directories"
	@echo "  make files   - Create files"
	@echo "  make clean   - Remove directories and files"
	@echo "  make help    - Show this help message"

.PHONY: all dirs files clean help