#import "template.typ": *

#let general = yaml({{SETTINGS_FILE_PATH}})
#let customers = yaml({{CUSTOMERS_FILE_PATH}})
#let context = yaml({{INVOICE_FILE_PATH}})

// Take a look at the file `template.typ` in the file panel
// to customize this template and discover how it works.
#show: project.with(
  title: context.title,
  date: datetime(day: int(context.date.day), month: int(context.date.month), 
  year: context.date.year),
  invoice_day_id: context.invoice_day_id,
  enterprise: general.enterprise,
  customer: customers.at(context.customer),
  lawRules: general.lawRules,
  politeness: general.politeness
)

#let products = context.products

#productsDetails(products, general.enterprise.tva)