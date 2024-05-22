#import "{{ TEMPLATE_PATH }}": *

#let general = yaml("{{ SETTINGS_PATH }}")
#let customers = yaml("{{ CUSTOMERS_PATH }}")
#let invoice_data = yaml("{{ INVOICE_PATH }}")

// Take a look at the file `template.typ` in the file panel
// to customize this template and discover how it works.
#show: project.with(
  title: invoice_data.title,
  date: datetime(day: int(invoice_data.date.day), month: int(invoice_data.date.month),
  year: int(invoice_data.date.year)),
  invoice_day_id: invoice_data.invoice_day_id,
  enterprise: general.enterprise,
  customer: customers.at(invoice_data.customer_id),
  lawRules: general.law_rules,
  politeness: general.politeness
)

#let products = invoice_data.products

#productsDetails(products, general.enterprise.tva)