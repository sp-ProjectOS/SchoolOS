import { NuxtAuthHandler } from '#auth'

const config = useRuntimeConfig()

export default NuxtAuthHandler({
	providers: [
		{
			id: 'ory',
			name: 'SchoolOS',
			type: 'oauth',
			authorization: {
				url: config.public.oauthUrl + '/oauth2/auth',
				params: {
					scope: 'openid profile email offline'
				}
			},
			token: config.public.oauthUrl + '/oauth2/token',
			userinfo: config.public.oauthUrl + '/userinfo',
			idToken: true,
			checks: ['pkce', 'state'],
			profile(profile) {
				return {
					id: profile.sub,
					name: profile.name,
					email: profile.email,
					image: profile.picture
				}
			}
		}
	]
})