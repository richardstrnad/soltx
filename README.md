# soltx - Simple Solana Transaction Parser

My main goal was to enable parsing of transactions that have not yet been
executed. For example, the ones visible from Phantom when developer mode is
activated. This functionality allows for a better understanding of which
programs and accounts are used.

## Usage

Run the program `soltx <TX>` where <TX> is the transaction as a simple String. 

```
soltx AW1xvM1m1s8IAgrytGTna+JRV+j+tKYOAmexnDBWATHJtMEcPL2KUtxaUx/Z8uRv8klQj877ojwUOflBgh7IuwmAAQADCpKmuLYQby0wrTzYFk3zHfBa772h5EuCr5y+eopA5GbZ2fOO1cbU7KGSrb5KYd+GxbKFXKWH6Idp0NoksJyQmUrKuHOr3Sgss16Uk4aKidr2YUeqXv4hg6yuUiOZpclKWcZODTl15g39TvNLR9E9XYnjHyPqgZCtLzwRaydKA/gE59gpzfVarX993DMjL+KBHL2O/MiH1LtUZrxLHbV5NO9rkgxC2x6B7MUm5yUGvP5pZrk6CPA28+n73qpCltlwiq81+3pgMgIIBJEVXUxl58P1RWn3fhSZ+KKhzhnr9WkuAwZGb+UhFzL/7K26csOb57yM5bvF9xJrLEObOkAAAAAEedVb8jHAbu50xW7OaBUH/bGy3qP0jlECsc2iVrwTj7Q/+if11/ZKdMCbHylYed5LCas238ndUUsyGqezjOXovEziZvivf8ewPvVm2cpQ4U/E5Yq38NCzDLi9korw3hUDBwAFAhEABgAHAAkDTQgAAAAAAAAIOR0AAQEIIQgJCB4KAQILDA0ODxAREhMAHx0IICcdHSgAGSIpAxoCGwQFBhwjJBQAAwEVFhcYHSUlJi3lF8uXeuOtKgMAAAATZAABLwAAZAECGWQCAE6OxDMAAAAATo7EMwAAAAAAAAADAMpe+YxLqLFxrHSbBEyJ1pcMgMltbhnJtuEVoOKw07EKqq6mr6OiqaXjpAQCT0es2j14F+PjU4b9FfN11+lCOP5AoYCnmRKMd7G8aIXwRx8FTlJRS0wGGjhQSk1PVs7CPi0DTKi6bknnldllxJq7AiZbJdxoZmAASqa+LcUE59zl6QPk5t8=

┌──────────────────────────────────────────────┐
│ Account                                      │
├──────────────────────────────────────────────┤
│ AsTySyN7EHM3RLpxeq5fgt9QLWRPsJ8W6mPAYCTmFZua │
│ Ffnsi9JamuLURJvQ7mWE5tt1Li4c6eSP3kwGbWEjAefK │
│ EeLV8r7KBqCVddmV2MWQnVW51qKG9FByiYCLf65KzwzG │
│ EM6khZwiyFYeZgMEaF1ZNwioRfMazte1WWheVkr9Qedq │
│ Gc2MYCCs5HD6xrrfsHxkv17cwh2rdrxnaR6SQW45o4sY │
│ 8EunmjSvpTrN8H426FUEgqbQFjK7VoGCYLBiXtwVW3a1 │
│ Cnx6BcBmBweZPbczi55vK33YNr8nAPQMgb1t7nVdaZVo │
│ ComputeBudget111111111111111111111111111111  │
│ JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4  │
│ D8cy77BBepLMngZx6ZukaTff5hCt1HrWyKk3Hnd9oitf │
└──────────────────────────────────────────────┘
┌─────────────────────────────────────────────┬────────────────────────────────────────────────────┐
│ Program ID                                  │                                               Data │
├─────────────────────────────────────────────┼────────────────────────────────────────────────────┤
│ ComputeBudget111111111111111111111111111111 │                                         2,17,0,6,0 │
│ ComputeBudget111111111111111111111111111111 │                                 3,77,8,0,0,0,0,0,0 │
│ JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4 │ 229,23,203,151,122,227,173,42,3,0,0,0,19,100,0,1,4 │
│                                             │ 7,0,0,100,1,2,25,100,2,0,78,142,196,51,0,0,0,0,78, │
│                                             │ 142,196,51,0,0,0,0,0,0,0                           │
└─────────────────────────────────────────────┴────────────────────────────────────────────────────┘
```
