# Container limits

simple test to check resource limits of a container



 ## Result 1
```yaml
limits:
  cpu: "2"
  memory: "2Gi"
requests:
  cpu: "1"
  memory: "1Gi"

```

```
------CPU and memory details from host 

Number of logical CPUs is 4
Total RAM: 16390360 KB
Free RAM: 7994912 KB
Used RAM: 6499412 KB

 ------ CPU and memory details from container using control groups v2 

CPU Weight: 39

CPU Max: 200000 100000

Memory Max: 2147483648

Current Memory Usage: 462848
2023-07-13T19:10:13.104838658Z
```


## Result 2

```yaml
limits:
  cpu: "2.5"
  memory: "2.5Gi"
requests:
  cpu: "1"
  memory: "1Gi"

```
 ------CPU and memory details from host 

Number of logical CPUs is 4
Total RAM: 16390360 KB
Free RAM: 8013324 KB
Used RAM: 6468524 KB

 ------ CPU and memory details from container using control groups v2 

CPU Weight: 39

CPU Max: 250000 100000

Memory Max: 2684354560

Current Memory Usage: 458752
2023-07-13T19:11:52.621864419Z
```