import React from "react"
import { Link } from "react-router-dom"
import styled from "styled-components"

export const Button = styled.button`
  display: inline-block;
  padding: 0.6rem 1.5rem;
  background: var(--gooey-brown);
  border-radius: var(--radius-small);
  color: var(--gooey-white);
  margin: 0.5rem;
  text-transform: uppercase;
  border: 1px solid var(--gooey-brown);
  font-weight: 700;
  font-size: 1.2rem;

  &:first-child {
    margin-left: 0;
  }

  &:last-child {
    margin-right: 0;
  }

  &:hover {
    background: var(--gooey-gold);
    border: 1px solid var(--gooey-gold);
    color: var(--gooey-mauve);
  }
`
const ButtonLinkInternal = Button.withComponent(Link)
const ButtonLinkExternal = Button.withComponent("a")

export function ButtonLink({
  to,
  children,
}: {
  to: string
  children: React.ReactNode
}) {
  if (to.match(/^[a-z/]+:/)) {
    return (
      <ButtonLinkExternal href={to} target="_blank">
        {children}
      </ButtonLinkExternal>
    )
  } else {
    return <ButtonLinkInternal to={to}>{children}</ButtonLinkInternal>
  }
}
