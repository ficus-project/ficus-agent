# Ficus Agent

**Agent used by ficus to fetch cloud resources and consumption, and store them as timeseries**  
*(work in progress)*  

## Providers

Providers have to be enabled by activating the corresponding features from the `Cargo.toml`.  
Several can be active at the same time.  

### AWS

To use the aws plugin, you need to have a AWS IAM user existing and configured into your `~/.aws/credentials` file, like this example:  
```toml
[default]
aws_access_key_id=YOUR-ACCESS-KEY
aws_secret_access_key=YOUR-SECRET-KEY
region=YOUR-REGION
```

> *Ficus doesn't need any write permission on AWS, so we recommend giving only readonly access on the resources you're interested in (like EC2, Cloudwatch)*  

### Mock

This plugin only serves test and showcase purpose.  

## Timeseries

The ficus agent currently only supports influxdb.  
Copy the `.influx.example.toml` config file into `.influx.toml`, and change with the proper values to connect to your influxdb instance.  

# Development

## Run locally

You'll need docker and the rust toolchain.  
- In the `local` folder, run `docker compose up -d` to start influxdb container
  - You need a `.influx.toml` config file; the values in `.influx.example.toml` match the local container default ones
- In the root project folder, run `cargo run`

## Project architecture

```mermaid
flowchart TD

  subgraph src
    main
    main --> analyzer
  end

  subgraph lib
    vm_provider
  end

  subgraph providers
    aws_vm_provider
    mock_vm_provider
  end

  influxdb[(influxdb)]
  aws((aws))

  analyzer --> vm_provider
  aws_vm_provider --> aws
  analyzer --> influxdb
  aws_vm_provider --> vm_provider
  mock_vm_provider --> vm_provider
  
```

*Providers are abstracted through the lib to ease multiple cloud providers implementation*  
