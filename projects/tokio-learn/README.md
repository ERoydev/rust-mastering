

1. This is simply first building i simple TCP client using STD that connect to my echo server `socat -v tcp-l:1234,fork exec:'/bin/cat'`
2. Then building that client using `tokio` 
3. Then building my own echo server using `tokio` instead of socat