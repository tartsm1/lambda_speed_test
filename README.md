# Lambda speed test to compare Rust and NodeJS

## Overview

Lambda speed test to compare Rust and NodeJS

## My test results 2023.12.08 (Amazon Linux AMI 2.0 )
### Coldstart:

NodeJS -Max Memory Used: 75 MB - Init Duration: 193.02 ms 

Rust - Max Memory Used: 13 MB - Init Duration: 22.29 ms 
### ReRun
NodeJS -Billed Duration: 305 ms - Max Memory Used: 76 MB

Rust - Billed Duration: 6 ms - Max Memory Used: 14 MB
 

## ⚠ Important

* Running this code might result in charges to your AWS account. For more details, see [AWS Pricing](https://aws.amazon.com/pricing/?aws-products-pricing.sort-by=item.additionalFields.productNameLowercase&aws-products-pricing.sort-order=asc&awsf.Free%20Tier%20Type=*all&awsf.tech-category=*all) and [Free Tier](https://aws.amazon.com/free/?all-free-tier.sort-by=item.additionalFields.SortRank&all-free-tier.sort-order=asc&awsf.Free%20Tier%20Types=*all&awsf.Free%20Tier%20Categories=*all).
* Running the tests might result in charges to your AWS account.
* We recommend that you grant your code least privilege. At most, grant only the minimum permissions required to perform the task. For more information, see [Grant least privilege](https://docs.aws.amazon.com/IAM/latest/UserGuide/best-practices.html#grant-least-privilege).
* This code is not tested in every AWS Region. For more information, see [AWS Regional Services](https://aws.amazon.com/about-aws/global-infrastructure/regional-product-services).

## Code examples

### Prerequisites

Environment variables should set:

**name - name of lambda to call**

**times - how many times repeat call**

Additionally, to compile Lambda functions written in the Rust programming language, use [Cargo Lambda](https://www.cargo-lambda.info/).

# Original code
https://github.com/awslabs/aws-lambda-rust-runtime/blob/main/examples/basic-lambda



