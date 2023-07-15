import { API_URL } from '$env/static/private';
import { error } from '@sveltejs/kit';
import axios from 'axios';
import * as jose from 'jose';
import moment from 'moment';

export const load = async ({ cookies }) => {
    const token = cookies?.get('jwt');
    if (!token) {
        throw error(401, 'Unauthorized');
    }

    const decoded: { sub: string, exp: number } = jose.decodeJwt(token) as any;
    if (decoded.exp < moment().unix()) {
        throw error(401, 'Unauthorized');
    }

    console.log(token)

    const followed = await axios.get(`${API_URL}/boards/feed`, {
        headers: {
            Authorization: `${token}`
        }
    });

    return {
        boards: followed.data
    }
}