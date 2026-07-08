import { Squirrel } from "lucide-solid";
import type { Component } from "solid-js";
import { css } from "styled-system/css";
import { VStack } from "styled-system/jsx";
import { Card, Heading, Text } from "~/components";
import { Container } from "../../../styled-system/jsx/container";

const iconClass = css({
	width: "{36}",
	height: "{36}",
	strokeWidth: "{1}",
});

export const NotFound: Component = () => {
	return (
		<Container p="12" maxW="md">
			<Card.Root>
				<Card.Body>
					<VStack gap="16">
						<VStack gap="0">
							<Heading textStyle="7xl">404</Heading>
							<Text textStyle="xl">Not Found</Text>
						</VStack>
						<VStack gap="4">
							<Squirrel class={iconClass} />
							<Text textStyle="md">
								We couldn't find what you were looking for, but we just found a
								cute squirrel.
							</Text>
						</VStack>
					</VStack>
				</Card.Body>
			</Card.Root>
		</Container>
	);
};
