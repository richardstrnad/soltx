alias d := dev
alias b := build
alias l := lint

build:
  @echo "Building..."
  cargo build --release
  @echo "Done."

@verify: test lint

test:
  cargo test

lint:
  cargo fmt --all -- --check
  cargo clippy

fmt:
  cargo fmt

dev:
  cargo watch -x "run --bin soltx AW1xvM1m1s8IAgrytGTna+JRV+j+tKYOAmexnDBWATHJtMEcPL2KUtxaUx/Z8uRv8klQj877ojwUOflBgh7IuwmAAQADCpKmuLYQby0wrTzYFk3zHfBa772h5EuCr5y+eopA5GbZ2fOO1cbU7KGSrb5KYd+GxbKFXKWH6Idp0NoksJyQmUrKuHOr3Sgss16Uk4aKidr2YUeqXv4hg6yuUiOZpclKWcZODTl15g39TvNLR9E9XYnjHyPqgZCtLzwRaydKA/gE59gpzfVarX993DMjL+KBHL2O/MiH1LtUZrxLHbV5NO9rkgxC2x6B7MUm5yUGvP5pZrk6CPA28+n73qpCltlwiq81+3pgMgIIBJEVXUxl58P1RWn3fhSZ+KKhzhnr9WkuAwZGb+UhFzL/7K26csOb57yM5bvF9xJrLEObOkAAAAAEedVb8jHAbu50xW7OaBUH/bGy3qP0jlECsc2iVrwTj7Q/+if11/ZKdMCbHylYed5LCas238ndUUsyGqezjOXovEziZvivf8ewPvVm2cpQ4U/E5Yq38NCzDLi9korw3hUDBwAFAhEABgAHAAkDTQgAAAAAAAAIOR0AAQEIIQgJCB4KAQILDA0ODxAREhMAHx0IICcdHSgAGSIpAxoCGwQFBhwjJBQAAwEVFhcYHSUlJi3lF8uXeuOtKgMAAAATZAABLwAAZAECGWQCAE6OxDMAAAAATo7EMwAAAAAAAAADAMpe+YxLqLFxrHSbBEyJ1pcMgMltbhnJtuEVoOKw07EKqq6mr6OiqaXjpAQCT0es2j14F+PjU4b9FfN11+lCOP5AoYCnmRKMd7G8aIXwRx8FTlJRS0wGGjhQSk1PVs7CPi0DTKi6bknnldllxJq7AiZbJdxoZmAASqa+LcUE59zl6QPk5t8="
