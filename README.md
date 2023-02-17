# readme

一个简易的服务器指标监控工具

## 使用步骤

1. 在需要监控的服务器上放置`machine_agent.exe`，并运行`machine_agent.exe`
2. 在观测服务器上放置`monitor.exe`与配置文件`config.yaml`
3. 在观测服务器上运行`monitor.exe`
4. 观测时会在`monitor.exe`生成`temp`文件夹并放入监控数据的csv文件
5. 观测结束后会在csv文件旁生成对应的可视化png图片

> 如果中途关闭`monitor.exe`，那么不会生成可视化png图片

## 配置文件说明

注意配置url时不要忘记`ws://`

```yaml
interval: 10 # 间隔时间，每个多少秒请求记录一次监控数据
targets:
  - url: "ws://example1.com:7878" # 需要监控的服务器1的ip和端口
    name: "Application Server 1" # 记录的文件名称标识
  - url: "ws://example2.com:7878" # 需要监控的服务器2的ip和端口
    name: "Application Server 2" # 记录的文件名称标识
duration: 300 # 持续时间，持续监控多少秒，到时后自动生成可视化文件并关闭monitor
```