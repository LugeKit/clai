# How to start

> Note that the project is based on volcengine platform.
> You need to create your access point and api key in [volcengine](https://www.volcengine.com/).

- Get your api_key from volcengine and export it to your shell

```bash
export LLM_KEY=${your_api_key}
```

- Set up your config.json in `~/.config/clai/config.json`

```json
{
  "prompt": "${your_prompt}",
  "timeout": 1800,
  "default_model": "doubao",
  "base_url": "ark.cn-beijing.volces.com/api/v3",
  "models": {
    "deepseek-r1": {
      "access_point": "${your_access_point1}"
    },
    "doubao": {
      "access_point": "${your_access_point2}"
    },
    "deepseek-r1-online": {
      "access_point": "${your_bot_access_point}",
      "base_url": "ark.cn-beijing.volces.com/api/v3/bots"
    }
  }
}
```

- Run clai in shell

```bash
clai -m deepseek-r1-online 'who are you'
```

- You can get help by

```bash
clai --help
```