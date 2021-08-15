NAME = npuzzle

all: build $(NAME)

build:
	cargo build --release

$(NAME):
	ln -s target/release/n-puzzle $(NAME)

clean:
	rm -rf target

fclean: clean
	rm -rf $(NAME)

re: fclean all

.PHONY: all build clean fclean re